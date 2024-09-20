using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class Fuel : UserControl
{
    private readonly Logic _logic = App.Logic;
    private ushort _lapCount;
    private int _previousFuelLevel = 100;
    
    public Fuel()
    {
        InitializeComponent();

        _logic.RegisterLowSpeedRefresh(Refresh);
    }

    private void Refresh()
    {
        var fuelLevel = _logic.FuelLevel;
        FuelLevel.Foreground = fuelLevel < Logic.FuelLevelAlertThrPct ? ColorPalette.Red : ColorPalette.White;
        FuelLevel.Text = fuelLevel.ToString();

        var lapCount = _logic.LapCount;
        if (_lapCount != lapCount)
        {
            var fuelConsumedThisLap = _previousFuelLevel - fuelLevel;
            var remainingLapsAtCurrentFuelConsumption = fuelLevel / fuelConsumedThisLap;
            
            _previousFuelLevel = fuelLevel;
            _lapCount = lapCount;

            FuelPerLap.Text = fuelConsumedThisLap.ToString("D2");
            LapsRemaining.Text = remainingLapsAtCurrentFuelConsumption.ToString("D2");
        }
    }
}