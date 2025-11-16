import java.util.Arrays;

public class removeDuplicatesFromSortedArray {
    public static void main(String[] args) {
        int[] arr = {0,0,1,1,1,2,2,3,3,4};
        int ans = removeDuplicates(arr);
        System.out.println("input arr: " + Arrays.toString(arr));

        System.out.println("ans : " + ans);
        for (int i = 0; i < ans; i++) {
            System.out.print(arr[i] + ", ");
        }
    }   
    public static int removeDuplicates(int[] nums) {
        int idx = 0;
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] != nums[idx]) {
                idx++;
                nums[idx] = nums[i];
            }
        }
        return idx+1;
    }
}
