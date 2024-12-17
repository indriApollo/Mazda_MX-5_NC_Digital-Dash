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

var stn = new Stn1110Simulator.Stn1110Simulator(portName, 115200);

Console.WriteLine("Running");
await stn.Run(cts.Token);
