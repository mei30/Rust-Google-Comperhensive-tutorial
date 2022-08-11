
pub mod merge_sort
{
    // TODO: design general quick sort using rust generics
    pub fn sort<T: std::cmp::PartialOrd + Copy>(items: &mut Vec<T>, p: usize, r: usize)
    {
        if p >= r
        {
            return;
        }
        
        let q: usize = (p + r) / 2;

        sort(items, p, q);
        sort(items, q + 1, r);
        merge(items, p, q, r)
    }

    fn merge<T: std::cmp::PartialOrd + Copy>(items: &mut Vec<T>, p: usize, q: usize, r: usize)
    {
        let n1 = q - p + 1;
        let n2 = r - q;
    
        let mut left_half_items: Vec<T> = Vec::new();
        let mut right_half_items: Vec<T> = Vec::new();

        for i in 0..n1
        {
            left_half_items.push(items[p + i]);
        }

        for i in 0..n2
        {
            right_half_items.push(items[q + 1 + i]);
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = p;

        while i < n1 && j < n2
        {
            if left_half_items[i] >= right_half_items[j]
            {
                items[k] = right_half_items[j];
                j += 1;
            }
            else {
                items[k] = right_half_items[i];
                i += 1
            }

            k += 1;
        }

        for z in i..n1
        {
            items[k] = left_half_items[z];
            k += 1;
        }

        for t in j..n2
        {
            items[k] = right_half_items[t];
            k += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
