using Avalonia;
using System;
using System.Linq;
using System.Threading;

namespace DigitalDash;

// ReSharper disable once ClassNeverInstantiated.Global
// ReSharper disable once ArrangeTypeModifiers
class Program
{
    public static bool SharedMemory { get; private set; } = false;

    // Initialization code. Don't use any Avalonia, third-party APIs or any
    // SynchronizationContext-reliant code before AppMain is called: things aren't initialized
    // yet and stuff might break.
    [STAThread]
    public static int Main(string[] args)
    {
        if (args.Contains("--shm"))
        {
            SharedMemory = true;
        }
        
        var builder = BuildAvaloniaApp();
        
        if (!args.Contains("--drm"))
            return builder.StartWithClassicDesktopLifetime(args);
        
        if (args.Contains("--silence-console"))
            SilenceConsole();
        
        // By default, Avalonia will try to detect output card automatically.
        // But you can specify one, for example "/dev/dri/card1".
        return builder.StartLinuxDrm(args: args, card: null, scaling: 1.0);

    }

    // Avalonia configuration, don't remove; also used by visual designer.
    // ReSharper disable once MemberCanBePrivate.Global
    public static AppBuilder BuildAvaloniaApp()
        => AppBuilder.Configure<App>()
            .UsePlatformDetect()
            .WithInterFont()
            .LogToTrace();
    
    private static void SilenceConsole()
    {
        // ReSharper disable once FunctionNeverReturns
        new Thread(() =>
            {
                Console.CursorVisible = false;
                while (true)
                    Console.ReadKey(true);
            })
            { IsBackground = true }.Start();
    }
}