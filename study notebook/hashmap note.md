# 哈希表語法使用筆記    
1.常用語法  
    (1). contains_key(&self, k: &K) -> bool
    (2). entry(&mut self, k: K) -> Entry<K, V>  
    (3). or_insert(&mut self, k: K, v: V) -> &mut V： 無須顯式聲明，rust 自動推斷value為可變
        let value = map.entry("key1").or_insert("default"); 
    (4). get(&self, k: &K) -> Option<&V>
    (5). get_mut(&mut self, k: &K) -> Option<&mut V>
        if let Some(value) = map.get_mut(&"key1") {
            *value += 1; // 在這裡解引用，來改變hashmap的value
        }
    注意：不能寫成，編譯器會報錯
    if let Some(&value) = map.get_mut(&"key1") {
     value += 1; 
    }
    (6).map.remove("key1") 刪除KEY
    (7).製作複雜結構的哈希表，如股發生兩次可變引用的問題，可以考慮使用 {} 来创建新的作用域。
    (8).map.len() 當前哈希表的鍵值對數量


2.枚舉型別Entry<K, V>  
pub enum Entry<'a, K: 'a, V: 'a> {  
    Occupied(OccupiedEntry<'a, K, V>),  
    Vacant(VacantEntry<'a, K, V>),  
}  
Occupied 表示 HashMap 中已经有一个键值对的条目，对应的类型是 OccupiedEntry。  
Vacant 表示 HashMap 中还没有对应键值对的条目，对应的类型是 VacantEntry。  

3.進階：    
    (1). 
    (2).    
    (3). 





