
#[cfg(test)]
mod tests {
    use fixed_point::Q7;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f3_to_q7() {
       let n1: f32 = 0.7;
       let q1 = Q7::from(n1);
       
       let n2 = -0.4;
       let q2 = Q7::from(n2);
       
       let n3 = 123.0;
       let q3 = Q7::from(n3);

       assert_eq!(q1, Q7(89));
       assert_eq!(q2, Q7(-51));
       assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);

        assert_eq!(n1, n2);
    }
}