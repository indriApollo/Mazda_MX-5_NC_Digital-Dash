if (args.Length != 1)
{
    Console.WriteLine("missing portName arg");
    return;
}

var portName = args[0];

var cts = new CancellationTokenSource();
Console.CancelKeyPress += (s, e) =>
{
    Console.WriteLine("Stopping...");
    cts.Cancel();
    e.Cancel = true;
};

var ubx = new UbloxSimulator.UbloxSimulator(portName, 38400);

Console.WriteLine("Running");
await ubx.Run(cts.Token);