namespace Xandos.ViewModels
{
    using Model;
    using System;
    using System.Diagnostics;
    using System.Windows.Input;

    public class MarkCommand : ICommand
    {
        public MarkCommand(IBoard board, int row, int column)
        {
            Debug.Assert(board != null);
            Debug.Assert(row >= 0 && row < 3);
            Debug.Assert(column >= 0 && column < 3);

            this.board = board;
            this.row = row;
            this.column = column;
        }

        public event EventHandler CanExecuteChanged;

        public bool CanExecute(object parameter) => board[row, column] == Sigil.Blank;

        public void Execute(object parameter) => board.MarkSquare(row, column);

        private readonly IBoard board;
        private readonly int column;
        private readonly int row;
    }
}