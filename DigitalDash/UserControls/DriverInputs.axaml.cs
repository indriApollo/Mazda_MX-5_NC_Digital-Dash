using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class DriverInputs : UserControl
{
    private readonly Logic _logic = App.Logic;
    
    public DriverInputs()
    {
        InitializeComponent();
        
        _logic.RegisterHighSpeedRefresh(Refresh);
    }

    private void Refresh()
    {
        Accelerator.Value = _logic.Accelerator;
        Throttle.Value = _logic.Throttle;
        EngineLoad.Value = _logic.EngineLoad;
        Brakes.Value = _logic.Brakes;
    }
}