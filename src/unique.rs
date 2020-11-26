pub fn unique<T>(xs: &mut Vec<T>)
where
    T: Ord,
{
    xs.sort();
    xs.dedup();
}
