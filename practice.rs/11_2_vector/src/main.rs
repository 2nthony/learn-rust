fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
    a7();
}

fn a1() {
    fn is_vec(_v: &Vec<u8>) {}

    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    // NOTE: also support () {}
    let v = vec![1, 2, 3];
    is_vec(&v);

    let mut v1 = Vec::new();
    for a in &arr {
        v1.push(*a);
    }

    is_vec(&v1);

    assert_eq!(v, v1);

    println!("1 ok")
}

fn a2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend([1, 2, 3]); // origin answer
                          // v2.extend_from_slice(&v1); my answer

    assert_eq!(v1, v2);

    println!("2 ok")
}

fn a3() {
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.to_vec();

    assert_eq!(v1, v2);

    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();

    assert_eq!(v1, v2);

    println!("3 ok");
}

fn a4() {
    let mut v = Vec::from([1, 2, 3]);

    for i in 0..5 {
        println!("{:?}", v.get(i));
    }

    for i in 0..5 {
        // match v.get(i) {
        //     Some(x) => v[i] = x + 1,
        //     None => v.push(i + 2),
        // }

        if let Some(x) = v.get(i) {
            v[i] = x + 1;
        } else {
            v.push(i + 2);
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("4 ok")
}

fn a5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[..v.len()];

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    // vec_ref.push(4); // also works
    (*vec_ref).push(4);

    let slice3 = &v[0..];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("5 ok")
}

fn a6() {
    let mut vec: Vec<i32> = Vec::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("6 ok")
}

fn a7() {
    #[derive(Debug, PartialEq)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    let arr = [
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(arr[1], IpAddr::V6("::1".to_string()));

    println!("7 ok")
}
