using System.IO.Ports;
using System.Text.RegularExpressions;
// ReSharper disable InconsistentNaming

namespace Stn1110Simulator;

public partial class Stn1110Simulator
{
    private readonly HashSet<string> _filteredCanIds = [];
    private readonly SerialPort _port;
    private bool _inMonitoringMode;
    private CancellationTokenSource _cts = null!;
    private Task _monitoringTask = null!;
    private readonly Random _random = new();

    public Stn1110Simulator(string portName, int baudRate)
    {
        _port = new SerialPort(portName, baudRate, Parity.None, 8, StopBits.One);
        _port.ReadTimeout = 1000;
        _port.WriteTimeout = 1000;
        _port.NewLine = "\r";
    }

    public async Task Run(CancellationToken cancellationToken)
    {
        _port.Open();
        
        while (!cancellationToken.IsCancellationRequested)
        {
            string line;
            try
            {
                line = ReadLine();
            }
            catch (TimeoutException)
            {
                continue;
            }

            if (_inMonitoringMode)
            {
                Console.WriteLine("Stopping Monitoring mode");
                await _cts.CancelAsync();
                _inMonitoringMode = false;
                await _monitoringTask;
                WriteLine("STOPPED");
                Write(">");
                continue;
            }
            
            if (line == "ATZ")
            {
                await RespondToATZ();
            }
            else if (line == "ATE0")
            {
                RespondToATE0();
            }
            else if (line == "ATH1")
            {
                RespondToATH1();
            }
            else if (line == "ATS0")
            {
                RespondToATS0();
            }
            else if (line.StartsWith("STFPA"))
            {
                HandleSTFPA(line);
            }
            else if (line == "STM")
            {
                HandleSTM();
            }
            else
            {
                WriteLine("?");
                Write(">");
            }
        }

        _port.Close();
    }

    private async Task RespondToATZ()
    {
        _filteredCanIds.Clear();
        await Task.Delay(TimeSpan.FromSeconds(1));
        WriteLine("ELM327 v1.5simu");
        Write(">");
        _port.DiscardInBuffer();
    }

    private void RespondToATE0()
    {
        WriteLine("OK");
        Write(">");
    }
    
    private void RespondToATH1()
    {
        WriteLine("OK");
        Write(">");
    }
    
    private void RespondToATS0()
    {
        WriteLine("OK");
        Write(">");
    }

    private void HandleSTFPA(string cmd)
    {
        var m = CanFilterRegex().Match(cmd);
        if (!m.Success)
        {
            WriteLine("ERROR");
            Write(">");
            return;
        }

        var filterCanId = m.Groups[1].Value;
        Console.WriteLine($"got filter can id {filterCanId}");
        _filteredCanIds.Add(filterCanId);
        WriteLine("OK");
        Write(">");
    }

    private void HandleSTM()
    {
        Console.WriteLine("Starting Monitoring mode");
        _cts = new CancellationTokenSource();
        _inMonitoringMode = true;
        _monitoringTask = SendMonitoringMessages(_cts.Token);
    }

    private async Task SendMonitoringMessages(CancellationToken cancellationToken)
    {
        Write("some garbage");
        
        while (!cancellationToken.IsCancellationRequested)
        {
            try
            {
                foreach (var id in _filteredCanIds)
                {
                    var data = _random.NextInt64();
                    Write(id);
                    await Task.Delay(3, cancellationToken); // simulate chopped transmission
                    WriteLine($"{data:X16}");
                }

                /*WriteLine("08571ED34B79EB53FC8");
                WriteLine("201725BB5766DC709FA");
                WriteLine("2405F4F8A4BC9876F4B");
                WriteLine("4302F95A180D3CBEE9E");
                WriteLine("4B029635C6448FBCD3D");*/

                
                await Task.Delay(100, cancellationToken);
            }
            catch (TaskCanceledException)
            {
            }
        }
    }

    private void WriteLine(string text)
    {
        Console.WriteLine($"out:{text}");
        _port.WriteLine(text);
    }

    private void Write(string text)
    {
        Console.WriteLine($"out:{text}");
        _port.Write(text);
    }

    private string ReadLine()
    {
        var line = _port.ReadLine();
        Console.WriteLine($"in:{line}"
            .Replace("\r", "[CR]")
            .Replace("\n", "[LF]"));
        return line;
    }

    [GeneratedRegex("STFPA([0-9A-F]{3}),[0-9A-F]{3}")]
    private static partial Regex CanFilterRegex();
}
