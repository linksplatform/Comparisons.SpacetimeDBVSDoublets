namespace Comparisons.SpacetimeDBVSDoublets;

public readonly record struct Link(ulong Id, ulong Source, ulong Target);

/// <summary>The same operations as the Rust Links trait; queries materialize their results.</summary>
public interface ILinks : IDisposable
{
    ulong Create(ulong source, ulong target);
    ulong CreatePoint();
    void Update(ulong id, ulong source, ulong target);
    void Delete(ulong id);
    void DeleteAll();
    Link[] QueryAll();
    Link? QueryById(ulong id);
    Link[] QueryBySource(ulong source);
    Link[] QueryByTarget(ulong target);
    Link[] QueryBySourceTarget(ulong source, ulong target);
    ulong Count();
}
