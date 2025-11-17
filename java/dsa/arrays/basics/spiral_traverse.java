package basics;

import java.util.Scanner;

public class spiral_traverse {
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

        System.out.println("Spiral Traversal:");
        spiral_traverse(matrix);
        
        sc.close();
    }
    
    public static void spiral_traverse(int[][] matrix) {
        int top = 0;
        int left = 0;
        int bottom = matrix.length - 1;
        int right = matrix[0].length - 1;

        int direction = 0;
        
        while (top <= bottom && left <= right) {
            if (direction == 0) {
                for (int i = left; i <= right; i++) {
                    System.out.print(matrix[top][i] + " -> ");
                }
                top++;
                direction++;
            }
            else if (direction == 1) {
                for (int i = top; i <= bottom; i++) {
                    System.out.print(matrix[i][right] + " -> ");
                }
                right--;
                direction++;
            }
            else if (direction == 2) {
                for (int i = right; i >= left; i--) {
                    System.out.print(matrix[bottom][i] + " -> ");
                }
                bottom--;
                direction++;
            }
            else if (direction == 3) {
                for (int i = bottom; i >= top; i--) {
                    System.out.print(matrix[i][left] + " -> ");
                }
                left++;
                direction = (direction + 1) % matrix.length;
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
