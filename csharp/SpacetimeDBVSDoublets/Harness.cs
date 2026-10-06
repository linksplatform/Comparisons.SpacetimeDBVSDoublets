using System.Diagnostics;
using System.Globalization;

namespace Comparisons.SpacetimeDBVSDoublets;

public enum SamplingMode { Auto, Linear, Flat }

/// <summary>
/// How a benchmark is sampled. The values match the Rust workflow,
/// and the iteration counts are chosen as Criterion 0.8 chooses them, so both languages measure the same way.
/// </summary>
/// <param name="SampleSize">Number of samples.</param>
/// <param name="Mode">Auto chooses flat sampling when a linear schedule would exceed twice the target time.</param>
/// <param name="WarmUp">How long the benchmark runs before it is measured.</param>
/// <param name="MeasurementTime">The measured time that all samples take together.</param>
public sealed record Sampling(int SampleSize, SamplingMode Mode, TimeSpan WarmUp, TimeSpan MeasurementTime)
{
    /// <summary>PR validation uses ten samples, one second warm-up and two seconds measurement.</summary>
    public static Sampling Quick { get; } = new(10, SamplingMode.Auto, TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(2));

    /// <summary>Full runs use twenty samples, three seconds warm-up and five seconds measurement.</summary>
    public static Sampling Full { get; } = new(20, SamplingMode.Auto, TimeSpan.FromSeconds(3), TimeSpan.FromSeconds(5));

    /// <summary>The number of iterations of every sample, for iterations that take <paramref name="nanoseconds"/> each.</summary>
    public long[] IterationCounts(double nanoseconds)
    {
        var measurement = MeasurementTime.TotalNanoseconds;
        var totalRuns = (double)SampleSize * (SampleSize + 1) / 2;
        var step = Math.Max(1, (long)Math.Ceiling(measurement / nanoseconds / totalRuns));
        if (Mode == SamplingMode.Flat || Mode == SamplingMode.Auto && totalRuns * step * nanoseconds > 2 * measurement)
        {
            var iterations = Math.Max(1, (long)Math.Ceiling(measurement / (nanoseconds * SampleSize)));
            return Enumerable.Repeat(iterations, SampleSize).ToArray();
        }
        return Enumerable.Range(1, SampleSize).Select(sample => sample * step).ToArray();
    }
}

/// <summary>The median time of one iteration and the standard deviation of the samples, in nanoseconds.</summary>
public readonly record struct Estimate(double Median, double StandardDeviation)
{
    /// <summary>Estimates the time of one iteration from the average iteration time of every sample.</summary>
    public static Estimate Of(IReadOnlyCollection<double> samples)
    {
        var sorted = samples.Order().ToArray();
        var middle = sorted.Length / 2;
        var median = sorted.Length % 2 == 0 ? (sorted[middle - 1] + sorted[middle]) / 2 : sorted[middle];
        var mean = sorted.Average();
        var variance = sorted.Length > 1 ? sorted.Sum(sample => (sample - mean) * (sample - mean)) / (sorted.Length - 1) : 0;
        return new Estimate(median, Math.Sqrt(variance));
    }

    /// <summary>
    /// The line that Criterion prints with <c>--output-format bencher</c>, for example
    /// <c>test create/SpacetimeDB/1000 ... bench:  44,055,505 ns/iter (+/- 5,345,991)</c>.
    /// </summary>
    public string Bencher(string group, string id) =>
        string.Create(CultureInfo.InvariantCulture,
            $"test {group}/{id} ... bench: {Math.Round(Median),11:N0} ns/iter (+/- {Math.Round(StandardDeviation):N0})");
}

public static class Harness
{
    /// <summary>
    /// Measures <paramref name="iteration"/>, which runs one iteration and returns its measured time:
    /// first warms up for <see cref="Sampling.WarmUp"/>, doubling the number of iterations each round,
    /// then takes <see cref="Sampling.SampleSize"/> samples.
    /// </summary>
    public static Estimate Measure(Sampling sampling, Func<TimeSpan> iteration)
    {
        TimeSpan Run(long iterations)
        {
            var elapsed = TimeSpan.Zero;
            for (long i = 0; i < iterations; i++)
            {
                elapsed += iteration();
            }
            return elapsed;
        }

        // Criterion calibrates iter_custom using wall time, which includes untimed setup.
        var warmUp = Stopwatch.StartNew();
        long warmUpIterations = 0;
        for (long iterations = 1; warmUp.Elapsed < sampling.WarmUp; iterations = checked(iterations * 2))
        {
            Run(iterations);
            warmUpIterations += iterations;
        }
        var counts = sampling.IterationCounts(Math.Max(1, warmUp.Elapsed.TotalNanoseconds / warmUpIterations));
        return Estimate.Of(counts.Select(count => Run(count).TotalNanoseconds / count).ToArray());
    }

}
