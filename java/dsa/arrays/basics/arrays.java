package basics;

import java.util.Arrays;
import java.util.Scanner;

public class arrays {
    public static void main(String[] args) {
        // =================================================== Arrays:

        // Arrays: Contiguous set of memory assigned to a contiguous data of same type.
        // Defining:
        int[] arr = new int[5];                         // creating an empty array with size : 5, where '0' as element by default
        int arr2[] = new int[5];                        // This definition is same as above.
        int arr3[] = {1,2,3,4,5};                       // Defining and Initialising array with values.
        
        // Indexing:
        // arr3 = 1  2  3  4  5
        // idx  = 0  1  2  3  4


        // Printing:
        System.out.println(Arrays.toString(arr));       // prints the array by converting it to the string using Arrays.toString(arr[]) inbuild function.
        System.out.println(Arrays.toString(arr2));       // prints the array by converting it to the string using Arrays.toString(arr[]) inbuild function.

        for (int i = 0; i < arr3.length; i++) {         // printing using the for loop (running for the length)
            System.out.print(arr[i]+" ");
        }

        // Taking user input for array:
        Scanner sc = new Scanner(System.in);

        System.out.print("\n\nEnter length of array to be defined:");
        int size = sc.nextInt();

        int arr4[] = new int[size];

        System.out.println("Enter values for array:");
        for (int i = 0; i < size; i++) {
            arr4[i] = sc.nextInt();
        }

        System.out.println("Array Created:");
        for (int i = 0; i < arr4.length; i++) {
            System.out.println("arr4[" + i + "] = " + arr4[i]);
        }


        // =================================================== 2-D Arrays:
        // 2-D arrays are like matrix : They are like array of arrays.
        // Its a nested structure.
        
        // Defining:
        int[][] matrix = new int[3][3];

        System.out.println("\n\nEnter values to be inserted in 3*3 matrix:");
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix.length; j++) {
                matrix[i][j] = sc.nextInt();
            }
        }

        System.out.println("\nMatrix Created:");
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix.length; j++) {
                System.out.print(matrix[i][j] + " ");
            }
            System.out.println();
        }

        System.out.println("\nIndex defined Matrix:");
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix.length; j++) {
                System.out.print("matrix[" + i + "][" + j + "] = " + matrix[i][j] + "   ");
            }
            System.out.println();
        }
        

        sc.close();
    }
}
