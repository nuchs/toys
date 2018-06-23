namespace Xandos.ViewModels
{
    using System.ComponentModel;

    public class ObservableObject : INotifyPropertyChanged
    {
        public event PropertyChangedEventHandler PropertyChanged;

        protected void OnPropertyChange(string property) => PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(property));
    }
}