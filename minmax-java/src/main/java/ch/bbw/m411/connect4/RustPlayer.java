package ch.bbw.m411.connect4;

public class RustPlayer extends Connect4ArenaMain.DefaultPlayer {
    private static native int rustPlay(byte player, byte[] board);
    private static native int rustBoardWinner(byte[] board);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("minmax_wrapper");
    }

    static byte[] encodeBoard(Connect4ArenaMain.Stone[] board) {
        byte[] boardBuf = new byte[board.length];
        for (int i = 0; i < board.length; i++) {
            var stone = board[i];
            byte value;
            if (stone == null) {
                value = 2;
            } else if (stone == Connect4ArenaMain.Stone.BLUE) {
                value = 1;
            } else {
                value = 0; // red
            }
            boardBuf[i] = value;
        }
        return boardBuf;
    }

    public static boolean isWinning(Connect4ArenaMain.Stone[] board, Connect4ArenaMain.Stone forColor) {
        byte[] boardBuf = RustPlayer.encodeBoard(board);
        byte expectedPlayer = switch (forColor) {
            case BLUE -> 1;
            case RED -> 0;
        };
        int winner = RustPlayer.rustBoardWinner(boardBuf);
        return winner == expectedPlayer;
    }

    @Override
    protected int play() {
        byte player = switch (this.myColor) {
            case BLUE -> 1;
            case RED -> 0;
        };
        byte[] boardBuf = RustPlayer.encodeBoard(this.board);
        return RustPlayer.rustPlay(player, boardBuf);
    }
}