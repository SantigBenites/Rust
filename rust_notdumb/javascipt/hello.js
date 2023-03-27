document.getElementsByClassName("myBtn").addEventListener("click", temp);


document.onload = function(){
    document.getElementsByClassName('myBtn').preventDefault();
}
  

function temp(){
    alert("hehe")
    window.location.reload(true);
}

function putEvent (){

    // Instantiating new EasyHTTP class
    const http = new EasyHTTP;
    // User Data
    const data = {
    	name: 'sunny yadav',
    	username: 'sunnyyadav',
    	email: 'sunny@gmail.com'
    }

    // Update Post
    http.put('http://127.0.0.1:8080/',data)

    // Resolving promise for response data
    .then(data => console.log(data))

    // Resolving promise for error
    .catch(err => console.log(err));

}

// ES6 class
class EasyHTTP {

    // Make an HTTP PUT Request
    async put(url, data) {
    
        // Awaiting fetch which contains method,
        // headers and content-type and body
        const response = await fetch(url, {
        method: 'PUT',
        headers: {
            'Content-type': 'application/json'
        },
        body: JSON.stringify(data)
        });
        
        // Awaiting response.json()
        const resData = await response.json();
    
        // Return response data
        return resData;
    }
}
    