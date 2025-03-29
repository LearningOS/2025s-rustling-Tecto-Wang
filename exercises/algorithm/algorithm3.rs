/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T>(array: &mut [T]){
    let n = array.len();
    // 冒泡排序：每一趟比较相邻元素，将较大的元素交换到后面
    for i in 0..n {
        // 提前退出标志：如果本趟没有发生交换，则数组已经有序
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}