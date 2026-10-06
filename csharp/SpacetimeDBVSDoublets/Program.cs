using Comparisons.SpacetimeDBVSDoublets;

var backend = args.FirstOrDefault(arg => arg.StartsWith("--backend="))?.Split('=')[1] ?? "all";
if (backend is not ("all" or "doublets" or "spacetimedb")) throw new ArgumentException("Invalid backend");
var quick = args.Contains("--quick");
int Size(string name, int fallback) => int.TryParse(Environment.GetEnvironmentVariable(name), out var value) && value > 0
    ? value : fallback;
var count = Size("BENCHMARK_LINK_COUNT", quick ? 10 : 1000);
var background = Size("BACKGROUND_LINK_COUNT", quick ? 30 : 3000);
var variants = (backend == "spacetimedb" ? Array.Empty<string>() : DoubletsLinks.Variants)
    .Concat(backend == "doublets" ? Array.Empty<string>() : ["SpacetimeDB"]);
var directory = Path.Combine(Path.GetTempPath(), $"spacetimedb-vs-doublets-{Guid.NewGuid()}");
Directory.CreateDirectory(directory);
try
{
    foreach (var variant in variants)
    {
        using ILinks store = variant == "SpacetimeDB" ? new SpacetimeLinks() : DoubletsLinks.Open(variant, directory);
        store.DeleteAll();
        foreach (var operation in Benchmarks.Operations)
        {
            var estimate = Harness.Measure(quick ? Sampling.Quick : Sampling.Full,
                () => Benchmarks.Iteration(store, operation, count, background));
            Console.WriteLine(estimate.Bencher(operation, $"{variant}/{count}"));
        }
    }
}
finally { Directory.Delete(directory, true); }
