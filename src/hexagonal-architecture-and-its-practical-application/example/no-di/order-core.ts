import { Db } from "./db-adapter"

export class Order {
  
  placeOrder(){
    Db.saveOrder({})
  }
}