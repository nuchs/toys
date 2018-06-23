namespace Xandos.ViewModels
{
    using Model;
    using System.Diagnostics;
    using System.Windows.Input;
    using System.Windows.Media;

    public class SquareViewModel : ObservableObject
    {
        public SquareViewModel(IBoard board, int row, int column)
        {
            Debug.Assert(board != null);

            Sigil = string.Empty;
            Background = new SolidColorBrush(Colors.White);
            MarkCommand = new MarkCommand(board, row, column);
            board.MonitorSquare(Update, row, column);
        }

        public Brush Background
        {
            get => background;
            set
            {
                background = value;
                OnPropertyChange(nameof(Background));
            }
        }

        public ICommand MarkCommand { get; }

        public string Sigil
        {
            get => sigil;
            set
            {
                if (sigil != value)
                {
                    sigil = value;
                    OnPropertyChange(nameof(Sigil));
                }
            }
        }

        private void Update(Sigil sigil, bool inWinningLine)
        {
            switch (sigil)
            {
                case Model.Sigil.Blank:
                    Sigil = "";
                    break;

                case Model.Sigil.Naught:
                    Sigil = "O";
                    break;

                case Model.Sigil.Cross:
                    Sigil = "X";
                    break;
            }

            if (inWinningLine)
            {
                Background = new SolidColorBrush(Colors.ForestGreen);
            }
        }

        private Brush background;
        private string sigil;
    }
}