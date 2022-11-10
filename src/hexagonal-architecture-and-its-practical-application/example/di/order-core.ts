import { OrderPort } from "./db-port"

export class Order {
  constructor(private Db: OrderPort) {
    this.Db = Db
  }

  placeOrder(){
    this.Db.saveOrder({})
  }
}