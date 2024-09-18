using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class Temperatures : UserControl
{
    private readonly Logic _logic = App.Logic;
    
    public Temperatures()
    {
        InitializeComponent();
        
        _logic.RegisterLowSpeedRefresh(Refresh);
    }

    private void Refresh()
    {
        var coolant = _logic.Coolant;
        Coolant.Foreground = coolant > Logic.CoolantAlertThrC ? ColorPalette.Red : ColorPalette.White;
        Coolant.Text = coolant.ToString();
        Intake.Text = _logic.Intake.ToString();
    }
}