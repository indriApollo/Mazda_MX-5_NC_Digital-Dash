using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;

namespace DigitalDash;

public class App : Application
{
    public static readonly Logic Logic = new(Program.UseMetricsShmClient, Program.UseChronoShmClient);
    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }

    public override void OnFrameworkInitializationCompleted()
    {
        switch (ApplicationLifetime)
        {
            case IClassicDesktopStyleApplicationLifetime desktop:
                desktop.MainWindow = new Views.MainWindow();
                break;
            case ISingleViewApplicationLifetime singleView:
                singleView.MainView = new Views.MainSingleView();
                break;
        }

        base.OnFrameworkInitializationCompleted();
    }
}