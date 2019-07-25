#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let num_nums = 10000;
        let mut nums = Vec::with_capacity(num_nums);
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            nums.push(rng.gen_range(0, num_nums * 5));
        }
        for n in crate::multi_sort(nums).iter() {
            println!("{}", n);
        }
    }
}

use std::sync::Arc;
use std::thread;

pub fn multi_sort<T: 'static + Clone + Ord + Send + Sync>(list: Vec<T>) -> Vec<T> {
    let threads = num_cpus::get();
    let division = list.len()/threads;
    let shared = Arc::new(list);
    let mut handles = Vec::with_capacity(threads);
    for i in 0..threads-1 {
        let s = shared.clone();
        handles.push(thread::spawn(move || {
            let start = i * division;
            sort(&s[start..start + division])
        }))
    }
    let endseg = division * (threads - 1);
    let mut out = sort(&shared[endseg..]);
    for handle in handles {
        out = merge(&handle.join().unwrap(), &out);
    }
    out
}

pub fn sort<T: Clone + Ord>(list: &[T]) -> Vec<T> {
    if list.len() == 1 {
        return vec!(list[0].clone());
    }
    if list.len() == 2 {
        if list[0] > list[1] {
            return vec!(list[1].clone(), list[0].clone());
        } else {
            return vec!(list[0].clone(), list[1].clone());
        }
    }
    let (a, b) = list.split_at(list.len()/2);
    merge(&sort(a), &sort(b))
}

fn merge<T: Clone + Ord>(mut a: &[T], mut b: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    loop {
        if a.len() == 0 {
            if b.len() == 0 {
                break;
            }
            for e in b.iter() {
                out.push(e.clone());
            }
            break;
        }
        if b.len() == 0 {
            for e in a.iter() {
                out.push(e.clone());
            }
            break;
        }
        if b[0] < a[0] {
            out.push(b[0].clone());
            b = &b[1..];
        } else {
            out.push(a[0].clone());
            a = &a[1..];
        }
    }
    out
}