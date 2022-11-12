import { Db } from "./db-adapter"

export class Order {
  private db: Db

  constructor(){
    this.db = new Db()
  }
  
  placeOrder(){
    this.db.saveOrder({})
  }
}