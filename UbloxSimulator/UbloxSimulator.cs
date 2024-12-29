using System.IO.Ports;

namespace UbloxSimulator;

public class UbloxSimulator
{
    private const byte UbxSyncChar1 = 0xb5;
    private const byte UbxSyncChar2 = 0x62;

    private enum UbxClass: byte{
        UbxNav = 0x01,
        UbxAck = 0x05,
        UbxCfg = 0x06,
        UbxMon = 0x0a
    };

    private enum UbxId: byte {
        UbxNavPosllh = 0x02,
        UbxNavStatus = 0x03,
        UbxMonVer = 0x04,
        UbxCfgValset = 0x8a
    };

    private enum UbxCfgKeyId: uint {
        CfgUsboutprotUbx = 0x10780001,
        CfgUsboutprotNmea = 0x10780002,
        CfgNavspgFixmode = 0x20110011,
        CfgNavspgDynmodel = 0x20110021,
        CfgMsgoutUbxNavPosllhUsb = 0x2091002c,
        CfgMsgoutUbxNavStatusUsb = 0x2091001d,
        CfgRateMeas = 0x30210001,
        CfgRateNav = 0x30210002
    };
    
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

            byte[] valset = [UbxSyncChar1, UbxSyncChar2, (byte)UbxClass.UbxCfg, (byte)UbxId.UbxCfgValset];
            if (data[..4].SequenceEqual(valset))
            {
                Console.WriteLine("got valset");
                byte[] ack = [UbxSyncChar1, UbxSyncChar2, (byte)UbxClass.UbxAck, 1, 2, 0, (byte)UbxClass.UbxCfg, (byte)UbxId.UbxCfgValset, 0x98, 0xc1];
                Write(ack);
            }
        }

        _port.Close();
    }
    

    private void Write(byte[] data)
    {
        Console.WriteLine($"out:{Convert.ToHexString(data)}");
        _port.Write(data, 0, data.Length);
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