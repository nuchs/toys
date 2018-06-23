namespace Tests
{
    using FluentAssertions;
    using Xandos.ViewModels;
    using Xunit;

    public class SquareViewModelTests
    {
        [Fact]
        public void ASquareInTheWinningRowShouldBeGreen()
        {
            var sut = new SquareViewModel(fakeBoard, 0, 0);

            fakeBoard.SetGameWon();
            sut.MarkCommand.Execute(null);

            sut.Background.ToString().Should().Be(green);
        }

        [Fact]
        public void MarkingTheSquareMeansItIsNotBlank()
        {
            var sut = new SquareViewModel(fakeBoard, 0, 0);

            sut.MarkCommand.Execute(null);

            sut.Sigil.Should().NotBe(string.Empty);
        }

        [Fact]
        public void SquareShouldInitiallyBeBlank()
        {
            var sut = new SquareViewModel(fakeBoard, 0, 0);

            sut.Sigil.Should().Be(string.Empty);
        }

        [Fact]
        public void SquaresShouldInitiallyBeWhite()
        {
            var sut = new SquareViewModel(fakeBoard, 0, 0);

            sut.Background.ToString().Should().Be(white);
        }

        private FakeBoard fakeBoard = new FakeBoard();
        private string green = "#FF228B22";
        private string white = "#FFFFFFFF";
    }
}