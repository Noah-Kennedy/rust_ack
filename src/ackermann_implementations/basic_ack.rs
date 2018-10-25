pub fn basic_ack(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        basic_ack(m - 1, 1)
    } else {
        basic_ack(m - 1, basic_ack(m, n - 1))
    }
}

#[cfg(test)]
mod tests {

}