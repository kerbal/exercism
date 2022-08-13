fn quick_sort<'a>(arr: &'a mut Vec<i32>, low: usize, high: usize) {
    println!("sort {} {}", low, high);
    let mid = (low + high) / 2;
    let mut l = low;
    let mut r = high;

    while l < r {
        while l <= high && arr[l] < arr[mid] { l += 1 }
        while r >= low && arr[r] > arr[mid] { r -= 1 }

        if l <= r {
            let t = arr[l];
            arr[l] = arr[r];
            arr[r] = t;
            l += 1;
            if r > 0 { r -= 1 }
        }

        if r > low { quick_sort(arr, low, r) }
        if l < high { quick_sort(arr, l, high) }
    }
}

fn bin_search <'a>(arr: &'a Vec<i32>, key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let k = (l + r) / 2;
        if arr[k] == key { return Some(k) }
        if arr[k] > key {
            if k > 0 {
                r = k - 1
            } else {
                return None
            }
        }
        if arr[k] < key { l = k + 1 }
    }

    None
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut arr_v: Vec<i32> = array.to_vec();
    let n = arr_v.len();
    if n == 0 { return None }
    arr_v.sort();
    // quick_sort(&mut arr_v, 0, n - 1);
    bin_search(&arr_v, key)
}
