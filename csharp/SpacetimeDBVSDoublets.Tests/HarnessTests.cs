using Comparisons.SpacetimeDBVSDoublets;
using Xunit;

namespace Comparisons.SpacetimeDBVSDoublets.Tests;

public sealed class HarnessTests
{
    [Fact]
    public void StatisticsMatchKnownSamples()
    {
        var estimate = Estimate.Of([10, 20, 30]);
        Assert.Equal(20, estimate.Median);
        Assert.Equal(10, estimate.StandardDeviation);
        Assert.Contains("test create/SpacetimeDB/10 ... bench:", estimate.Bencher("create", "SpacetimeDB/10"));
        Assert.Contains("ns/iter (+/- 10)", estimate.Bencher("create", "SpacetimeDB/10"));
        Assert.Equal(25, Estimate.Of([10, 20, 30, 40]).Median);
    }

    [Fact]
    public void SamplingUsesCriterionLinearIterationCounts()
    {
        var sampling = new Sampling(10, SamplingMode.Linear, TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(2));
        Assert.Equal(Enumerable.Range(1, 10).Select(i => i * 37L), sampling.IterationCounts(1_000_000));
    }

    [Fact]
    public void SlowIterationsUseCriterionAutomaticFlatSampling()
    {
        Assert.Equal(Enumerable.Repeat(2L, 10), Sampling.Quick.IterationCounts(100_000_000));
    }
}
