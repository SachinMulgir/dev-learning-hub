package basics;

import java.util.Scanner;

public class zigzag_traversal {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter size for matrix:");
        int out_size = sc.nextInt();
        int in_size = sc.nextInt();

        int[][] matrix = new int[out_size][in_size];

        System.out.println("\nEnter values to be inserted in matrix:");
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                matrix[i][j] = sc.nextInt();
            }
        }
        
        System.out.println("\nInput Matrix:");
        print_matrix(matrix);

        System.out.println("\nZig Zag Traversal with Skip:");
        zigzag_traversal(matrix);

        sc.close();
    }

    public static void zigzag_traversal(int[][] matrix) {
        boolean toggle = true;
        int rows = matrix.length;
        int cols = matrix[0].length;

        for (int i = 0; i < rows; i++) {
            if (i%2==0) {
                for (int j = 0; j < cols; j++) {
                    if (toggle) System.out.print(matrix[i][j] + " ");
                    toggle = toggle ? false : true;
                }
            }
            else {
                for (int j = cols-1; j >= 0; j--) {
                    if (toggle) System.out.print(matrix[i][j] + " ");
                    toggle = toggle ? false : true;
                }
            }
        }
    }

    public static void print_matrix(int[][] matrix) {
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                System.out.print(matrix[i][j] + " ");
            }
            System.out.println();
        }
    }
}
