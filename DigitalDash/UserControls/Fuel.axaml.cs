using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class Fuel : UserControl
{
    private readonly Logic _logic = App.Logic;
    
    public Fuel()
    {
        InitializeComponent();

        _logic.RegisterLowSpeedRefresh(Refresh);
    }

    private void Refresh()
    {
        var fuelGauge = _logic.FuelGauge;
        FuelGauge.Foreground = fuelGauge < Logic.FuelGaugeAlertThrPct ? ColorPalette.Red : ColorPalette.White;
        FuelGauge.Text = fuelGauge.ToString();
    }
}