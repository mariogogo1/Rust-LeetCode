# 單調棧使用筆記    
建造嚴格遞減棧：  
1.方法：  
    棧頂元素(top) <= 入棧元素(element)，pop(top)彈出棧頂元素。  
    直到棧頂元素(top) > 入棧元素(element)，或是is_empty()棧沒有元素了,push(element)。 

2.功能：  
    (1).找出第一個比top大的元素，當element pop top ，element就是比top大的第一個元素  
    (2).找出在element前面同時比element更大的元素，element入棧後，element.prev()就是前面第一個大於element的元素。  
      
3.常用語法  
stack.pop();  
stack.first();  
stack.last();  
stack.push(x);  
while let Some(&top) = stack.last(){
    ....
};  
if let Some(&top) = stack.last(){
    ....
}else{

};  
stack.is_empty();  

4.進階：    
    (1). 雙重單調棧：找到兩個大於element的元素；參考2454題
    (2). 除了第一個element外，每個element入棧時遇到的top都是原本element的前一個元素。   
    (3). 有時候兩個element相等可以無視，改用特別用hashmap紀錄；參考975題。  





