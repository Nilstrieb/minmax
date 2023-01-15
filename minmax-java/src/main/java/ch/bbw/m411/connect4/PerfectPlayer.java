package ch.bbw.m411.connect4;

import java.util.ArrayList;
import java.util.List;

public class PerfectPlayer extends Connect4ArenaMain.DefaultPlayer {
    private static final int MAX_DEPTH = 8;
    private static final int WON = 1000;
    private static final int LOST = -1000;

    private int bestMove = -1;

    private List<Integer> possibleMoves() {
        var moves = new ArrayList<Integer>();
        // for all columns
        for (int i = 0; i < 7; i++) {
            // walk up
            for (int j = 0; j < 4; j++) {
                var position = i + (j * 7);
                if (board[position] == null) {
                    moves.add(position);
                    break;
                }
            }

        }
        return moves;
    }

    private int minmax(Connect4ArenaMain.Stone maximizingPlayer, int alpha, int beta, int depth) {
        if (depth >= MAX_DEPTH) {
            // FIXME rate
            return 0;
        }

        if (Connect4ArenaMain.isWinning(board, maximizingPlayer)) {
            return WON;
        }

        if (Connect4ArenaMain.isWinning(board, maximizingPlayer.opponent())) {
            return LOST;
        }

        var moves = possibleMoves();
        if (moves.isEmpty()) {
            return 0;
        }

        var maxValue = alpha;
        for (var move : moves) {
            board[move] = maximizingPlayer;
            var value = -minmax(maximizingPlayer.opponent(), -beta, -maxValue, depth + 1);
            board[move] = null;

            if (value > maxValue) {
                maxValue = value;
                if (depth == 0) {
                    bestMove = move;
                }

                if (maxValue >= beta) {
                    break;
                }
            }
        }

        return maxValue;
    }

    @Override
    protected int play() {
        bestMove = -1;
        minmax(myColor, LOST, WON, 0);
        return this.bestMove;
    }
}
