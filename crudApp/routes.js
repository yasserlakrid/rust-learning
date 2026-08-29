async function  updateTask(id , title , status){ 
    return await fetch(`http://127.0.0.1:7777/tasks/${id}` , {
        method : 'PUT' , 
        headers : {
                "content-type" : "application/json"
            }
        , 
        body : JSON.stringify({
            title : title , done : status
        })}
        ).then (res => res.json()).then(res => res).catch(err => console.log(err))
}
async function addTask(title) {
    return await fetch("http://127.0.0.1:7777/tasks" , {
method : 'POST' , headers : {
  'content-type' : 'application/json'
},
body : JSON.stringify({title : title})})
.then(res => res.json())
.then(data => console.log(data)).catch(err => console.log(err))
}
async function getTasks() {
    return await fetch("http://127.0.0.1:7777/tasks" , {
    method:'GET' , 
    headers:{
        'content-type' : 'application/json',
    }
    }).then(res=>res.json()).then(res => console.log(res)).catch(err => console.log(err))
}
