using System.IO.Ports;

namespace UbloxSimulator;

public class UbloxSimulator
{
    private readonly SerialPort _port;
    
    public UbloxSimulator(string portName, int baudRate)
    {
        _port = new SerialPort(portName, baudRate, Parity.None, 8, StopBits.One);
        _port.ReadTimeout = 1000;
        _port.WriteTimeout = 1000;
    }
    
    public async Task Run(CancellationToken cancellationToken)
    {
        _port.Open();
        
        while (!cancellationToken.IsCancellationRequested)
        {
            byte[] data;
            try
            {
                data = Read();
            }
            catch (TimeoutException)
            {
                continue;
            }

            //
        }

        _port.Close();
    }
    

    private void Write(string text)
    {
        Console.WriteLine($"out:{text}");
        _port.Write(text);
    }

    private byte[] Read()
    {
        var data = new byte[128];
        var c = _port.Read(data, 0, data.Length);
        Array.Resize(ref data, c);
        Console.WriteLine($"in:{Convert.ToHexString(data)}");
        return data;
    }
}