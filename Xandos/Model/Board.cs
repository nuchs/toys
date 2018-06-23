namespace Model
{
    using System;
    using System.Collections.Generic;
    using System.Diagnostics;
    using System.Linq;

    public class Board : IBoard
    {
        public Board()
        {
            CreateBoard();
            SetLines();
        }

        public bool GameEnded { get; private set; }

        public Sigil this[int row, int column]
        {
            get
            {
                CheckIndiciesValid(row, column);

                return board[row, column].Sigil;
            }
        }

        public void MarkSquare(int row, int column)
        {
            if (GameEnded)
            {
                return;
            }

            CheckMoveIsValid(row, column);

            board[row, column].Sigil = nextMove;

            ToggleNextMoveSigil();
            EvaluateBoard();
        }

        public void MonitorSquare(Action<Sigil, bool> callback, int row, int column) => board[row, column].Notify += callback;

        [Conditional("DEBUG")]
        private void CheckIndiciesValid(int row, int column)
        {
            Debug.Assert(row >= 0 && row < BoardSize);
            Debug.Assert(column >= 0 && column < BoardSize);
        }

        [Conditional("DEBUG")]
        private void CheckMoveIsValid(int row, int column)
        {
            CheckIndiciesValid(row, column);
            Debug.Assert(board[row, column].Sigil == Sigil.Blank);
        }

        private void CreateBoard()
        {
            for (var row = 0; row < BoardSize; row++)
            {
                for (var column = 0; column < BoardSize; column++)
                {
                    board[row, column] = new Square(row, column);
                }
            }
        }

        private void EvaluateBoard()
        {
            HasGameEnded();

            foreach (var line in lines)
            {
                if (IsWinning(line))
                {
                    foreach (var square in line)
                    {
                        square.InWinningLine();
                    }

                    GameEnded = true;

                    break;
                }
            }
        }

        private void HasGameEnded() => GameEnded = (from Square square in board select square.Sigil).All(s => s != Sigil.Blank);

        private bool IsWinning(List<Square> line) => line[0].Sigil != Sigil.Blank &&
                                                     line[0].Sigil == line[1].Sigil &&
                                                     line[0].Sigil == line[2].Sigil;

        private void SetLines() => lines = new List<List<Square>>
            {
                new List<Square> { board[0,0], board[0, 1], board[0,2] },
                new List<Square> { board[1,0], board[1, 1], board[1,2] },
                new List<Square> { board[2,0], board[2, 1], board[2,2] },

                new List<Square> { board[0,0], board[1, 0], board[2,0] },
                new List<Square> { board[0,1], board[1, 1], board[2,1] },
                new List<Square> { board[0,2], board[1, 2], board[2,2] },

                new List<Square> { board[0,0], board[1, 1], board[2,2] },
                new List<Square> { board[0,2], board[1, 1], board[2,0] },
            };

        private void ToggleNextMoveSigil() => nextMove = (nextMove == Sigil.Cross) ? Sigil.Naught : Sigil.Cross;

        private const int BoardSize = 3;
        private Square[,] board = new Square[BoardSize, BoardSize];
        private List<List<Square>> lines;
        private Sigil nextMove = Sigil.Cross;
    }
}