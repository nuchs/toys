namespace Tests
{
    using FluentAssertions;
    using Model;
    using System.Collections.Generic;
    using Xunit;

    public class BoardTests
    {
        [Theory]
        [MemberData(nameof(TestData.TryAllSquares), MemberType = typeof(TestData))]
        public void AMarkedSquareShouldNotBeBlank(int row, int column)
        {
            sut.MarkSquare(row, column);
            sut[row, column].Should().NotBe(Sigil.Blank);
        }

        [Fact]
        public void CrossesShouldFollowANaught()
        {
            sut.MarkSquare(0, 0);
            sut.MarkSquare(0, 1);
            sut.MarkSquare(0, 2);

            sut[0, 2].Should().Be(Sigil.Cross);
        }

        [Fact]
        public void CrossesShouldGoFirst()
        {
            sut.MarkSquare(0, 0);

            sut[0, 0].Should().Be(Sigil.Cross);
        }

        [Theory]
        [MemberData(nameof(TestData.TryAllSquares), MemberType = typeof(TestData))]
        public void MarkingOneSquareShouldNotMarkAnother(int row, int column)
        {
            sut.MarkSquare(row, column);

            for (int i = 0; i < 3; i++)
            {
                for (int j = 0; j < 3; j++)
                {
                    if (i != row && j != column)
                    {
                        sut[i, j].Should().Be(Sigil.Blank);
                    }
                }
            }
        }

        [Fact]
        public void NaughtsShouldFollowACross()
        {
            sut.MarkSquare(0, 0);
            sut.MarkSquare(0, 1);

            sut[0, 1].Should().Be(Sigil.Naught);
        }

        [Theory]
        [MemberData(nameof(TestData.TryAllSquares), MemberType = typeof(TestData))]
        public void SubscribersShouldBeNotifiedWhenASquareChanges(int row, int column)
        {
            var marked = false;

            sut.MonitorSquare((_, __) => marked = true, row, column);
            sut.MarkSquare(row, column);

            marked.Should().BeTrue();
        }

        [Theory]
        [MemberData(nameof(TestData.TryAllWinningLines), MemberType = typeof(TestData))]
        public void SubscribersShouldBeNotifiedWhenSquaresFormAWinningLine((int, int) p1_first, (int, int) p2_first, (int, int) p1_second, (int, int) p2_second, (int, int) p1_third)
        {
            var winner1 = false;
            var winner2 = false;
            var winner3 = false;

            sut.MonitorSquare((_, win) => winner1 = win, p1_first.Item1, p1_first.Item2);
            sut.MonitorSquare((_, win) => winner2 = win, p1_second.Item1, p1_second.Item2);
            sut.MonitorSquare((_, win) => winner3 = win, p1_third.Item1, p1_third.Item2);

            sut.MarkSquare(p1_first.Item1, p1_first.Item2);
            sut.MarkSquare(p2_first.Item1, p2_first.Item2);
            sut.MarkSquare(p1_second.Item1, p1_second.Item2);
            sut.MarkSquare(p2_second.Item1, p2_second.Item2);
            sut.MarkSquare(p1_third.Item1, p1_third.Item2);

            winner1.Should().BeTrue();
            winner2.Should().BeTrue();
            winner3.Should().BeTrue();
        }

        [Theory]
        [MemberData(nameof(TestData.TryAllSquares), MemberType = typeof(TestData))]
        public void TheBoardShouldStartOffBlank(int row, int column)
        {
            sut[row, column].Should().Be(Sigil.Blank);
        }

        [Fact]
        public void TheGameIsOverAfterAWinningMoveIsMade()
        {
            sut.MarkSquare(0, 0);
            sut.MarkSquare(1, 1);
            sut.MarkSquare(0, 1);
            sut.MarkSquare(2, 2);
            sut.MarkSquare(0, 2);

            sut.GameEnded.Should().BeTrue();
        }

        [Fact]
        public void TheGameShouldNotInitiallyBeOver()
        {
            sut.GameEnded.Should().BeFalse();
        }

        [Fact]
        public void TheGameIsOverIfNoMoreMovesCanBeMade()
        {
            sut.MarkSquare(0, 0);
            sut.MarkSquare(0, 1);
            sut.MarkSquare(0, 2);
            sut.MarkSquare(1, 0);
            sut.MarkSquare(1, 1);
            sut.MarkSquare(2, 2);
            sut.MarkSquare(1, 2);
            sut.MarkSquare(2, 0);
            sut.MarkSquare(2, 1);

            sut.GameEnded.Should().BeTrue();
        }

        [Fact]
        public void MovesMadeAfterTheGameHasEndedAreIgnored()
        {
            sut.MarkSquare(0, 0);
            sut.MarkSquare(1, 0);
            sut.MarkSquare(0, 1);
            sut.MarkSquare(1, 1);
            sut.MarkSquare(0, 2);
            sut.MarkSquare(2, 2);

            sut[2, 2].Should().Be(Sigil.Blank);
        }

        private IBoard sut = new Board();

        private static class TestData
        {
            public static IEnumerable<object[]> TryAllSquares()
            {
                yield return new object[] { 0, 0 };
                yield return new object[] { 0, 1 };
                yield return new object[] { 0, 2 };
                yield return new object[] { 1, 0 };
                yield return new object[] { 1, 1 };
                yield return new object[] { 1, 2 };
                yield return new object[] { 2, 0 };
                yield return new object[] { 2, 1 };
                yield return new object[] { 2, 2 };
            }

            public static IEnumerable<object[]> TryAllWinningLines()
            {
                var stubA = (0, 0);
                var stubB = (1, 1);
                var stubC = (2, 2);
                var stubD = (1, 2);
                var stubE = (0, 2);

                yield return new object[] { (0, 0), stubB, (0, 1), stubC, (0, 2) };
                yield return new object[] { (1, 0), stubA, (1, 1), stubC, (1, 2) };
                yield return new object[] { (2, 0), stubA, (2, 1), stubB, (2, 2) };

                yield return new object[] { (0, 0), stubB, (1, 0), stubC, (2, 0) };
                yield return new object[] { (0, 1), stubA, (1, 1), stubC, (2, 1) };
                yield return new object[] { (0, 2), stubA, (1, 2), stubB, (2, 2) };

                yield return new object[] { (0, 0), stubD, (1, 1), stubE, (2, 2) };
                yield return new object[] { (0, 2), stubA, (1, 1), stubC, (2, 0) };
            }
        }
    }
}