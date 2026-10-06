using System.Diagnostics;
using SpacetimeDB;
using SpacetimeDB.Types;
using DbLink = SpacetimeDB.Types.Link;

namespace Comparisons.SpacetimeDBVSDoublets;

/// <summary>Official C# SDK: reducer writes and reads from the subscribed client cache.</summary>
public sealed class SpacetimeLinks : ILinks
{
    private readonly DbConnection connection;
    private bool completed;
    private Exception? failure;

    public SpacetimeLinks()
    {
        var ready = false;
        var connected = false;
        connection = DbConnection.Builder()
            .WithUri(Environment.GetEnvironmentVariable("SPACETIMEDB_URI") ?? "http://localhost:3000")
            .WithDatabaseName(Environment.GetEnvironmentVariable("SPACETIMEDB_DB") ?? "benchmark-links")
            .OnConnect((_, _, _) => connected = true)
            .OnConnectError(error => failure = error)
            .OnDisconnect((_, error) => failure = error ?? new IOException("SpacetimeDB disconnected"))
            .Build();
        connection.Reducers.OnCreateLink += (context, _, _) => Complete(context);
        connection.Reducers.OnUpdateLink += (context, _, _, _) => Complete(context);
        connection.Reducers.OnDeleteLink += (context, _) => Complete(context);
        connection.Reducers.OnDeleteAllLinks += Complete;
        try
        {
            PumpUntil(() => connected);
            connection.SubscriptionBuilder()
                .OnApplied(_ => ready = true)
                .OnError((_, error) => failure = error)
                .Subscribe(["SELECT * FROM links"]);
            PumpUntil(() => ready);
        }
        catch
        {
            connection.Disconnect();
            throw;
        }
    }

    private void Complete(ReducerEventContext context)
    {
        switch (context.Event.Status)
        {
            case Status.Failed(var reason): failure = new InvalidOperationException(reason); break;
            case Status.OutOfEnergy: failure = new InvalidOperationException("SpacetimeDB out of energy"); break;
        }
        completed = true;
    }

    private void PumpUntil(Func<bool> done)
    {
        var elapsed = Stopwatch.StartNew();
        while (!done())
        {
            connection.FrameTick();
            if (failure is not null) throw new InvalidOperationException("SpacetimeDB operation failed", failure);
            if (elapsed.Elapsed > TimeSpan.FromSeconds(30)) throw new TimeoutException("SpacetimeDB operation timed out");
            Thread.Yield();
        }
        if (failure is not null) throw new InvalidOperationException("SpacetimeDB operation failed", failure);
    }

    private void Invoke(Action reducer)
    {
        completed = false;
        reducer();
        PumpUntil(() => completed);
    }

    private static Link Convert(DbLink row) => new(row.Id, row.Source, row.Target);
    public ulong Create(ulong source, ulong target)
    {
        Invoke(() => connection.Reducers.CreateLink(source, target));
        return connection.Db.Links.Iter().Where(row => row.Source == source && row.Target == target).Max(row => row.Id);
    }
    public ulong CreatePoint()
    {
        var id = Create(0, 0);
        Update(id, id, id);
        return id;
    }
    public void Update(ulong id, ulong source, ulong target) => Invoke(() => connection.Reducers.UpdateLink(id, source, target));
    public void Delete(ulong id) => Invoke(() => connection.Reducers.DeleteLink(id));
    public void DeleteAll() => Invoke(connection.Reducers.DeleteAllLinks);
    public Link[] QueryAll() => connection.Db.Links.Iter().Select(Convert).ToArray();
    public Link? QueryById(ulong id) => connection.Db.Links.Iter().Where(row => row.Id == id).Select(row => (Link?)Convert(row)).FirstOrDefault();
    public Link[] QueryBySource(ulong source) => connection.Db.Links.Iter().Where(row => row.Source == source).Select(Convert).ToArray();
    public Link[] QueryByTarget(ulong target) => connection.Db.Links.Iter().Where(row => row.Target == target).Select(Convert).ToArray();
    public Link[] QueryBySourceTarget(ulong source, ulong target) => connection.Db.Links.Iter()
        .Where(row => row.Source == source && row.Target == target).Select(Convert).ToArray();
    public ulong Count() => (ulong)connection.Db.Links.Count;
    public void Dispose() => connection.Disconnect();
}
