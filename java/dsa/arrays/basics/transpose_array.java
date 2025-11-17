package basics;
import java.util.Scanner;

public class transpose_array {
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

        // traverse
        // 1 2 3            1 4 7
        // 4 5 6     -->    2 5 8
        // 7 8 9            3 6 9

        // funtion to transpose matrix:
        int[][] traversed_matrix = transpose(matrix);


        System.out.println("\nTraversed Matrix:");
        print_matrix(traversed_matrix);
        
        
        sc.close();
    }
    public static int[][] transpose(int[][] matrix) {
        int[][] ans_mat = new int[matrix[0].length][matrix.length];
        for(int i = 0; i < matrix.length; i++) {
            for(int j = 0; j < matrix[i].length; j++) {
                ans_mat[j][i] = matrix[i][j];
            }
        }
        return ans_mat;
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
