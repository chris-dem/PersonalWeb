import React, { useEffect, useState } from 'react';
import {  Either, fold, left, right } from 'fp-ts/lib/Either';

interface Data {
    userId : number,
    userName : string
}

export const Testing : React.FC = () => {
    const [req,setReq] = useState<Either<Boolean,Data[]>>(left(false));
    useEffect(() => {
        fetch("http://localhost:5000/users")
        .then(res => {
            console.log(res)
            return res.json()
        })
        .then(
            (result) => {
                console.log(result)
                setReq(right(result))
            },
            (error) => {
                console.log(error)
                setReq(left(true))
            }
        )
    },[])
    let func = fold((e) => {
        return (<div> {e ? "Error : failed to fetch" : "Loading.."} </div>)
    }, (items : Data[]) => {
       return (
           <ul>
               {items.map((item) => (
                   <li key={item.userId}>
                       {item.userName}
                   </li>    
               ))}
           </ul>
       )  
    })
    return func(req)
}