# egui_dnd (tweaked)
... is a drag & drop library for [egui](https://github.com/emilk/egui). 

Give it a try here: https://lucasmerlin.github.io/egui_dnd/

**Tweaks:**
- automatic drag handle offset correction   
  works only if drag item layout directly contains the drag handle (no nesting)
- manual drag handle offset correction   
  User provides the offset of the drag handle relative to the drag item layout


To get started, take a look at the [simple example.](https://github.com/lucasmerlin/egui_dnd/blob/main/examples/simple.rs)



![ezgif-2-41c0c5360f](https://user-images.githubusercontent.com/8009393/208403722-b28715cd-b708-4eb4-8d00-36873dee2034.gif)

![custom drag handle offset](https://user-images.githubusercontent.com/6931775/209220847-cf58b4ed-3ed4-4f5a-8bfa-ad0ada4a5616.gif)