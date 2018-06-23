namespace Xandos.ViewModels
{
    using Model;
    using System.Diagnostics;

    internal class MainWindowViewModel
    {
        internal MainWindowViewModel(IBoard board)
        {
            Debug.Assert(board != null);

            Bottom = new SquareViewModel(board, 2, 1);
            BottomLeft = new SquareViewModel(board, 2, 0);
            BottomRight = new SquareViewModel(board, 2, 2);
            Left = new SquareViewModel(board, 1, 0);
            Middle = new SquareViewModel(board, 1, 1);
            Right = new SquareViewModel(board, 1, 2);
            Top = new SquareViewModel(board, 0, 0);
            TopLeft = new SquareViewModel(board, 0, 1);
            TopRight = new SquareViewModel(board, 0, 2);
        }

        public SquareViewModel Bottom { get; }
        public SquareViewModel BottomLeft { get; }
        public SquareViewModel BottomRight { get; }
        public SquareViewModel Left { get; }
        public SquareViewModel Middle { get; }
        public SquareViewModel Right { get; }
        public SquareViewModel Top { get; }
        public SquareViewModel TopLeft { get; }
        public SquareViewModel TopRight { get; }
    }
}