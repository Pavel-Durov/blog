import { OrderPort } from "./db-port";

export class Db implements OrderPort {
  saveOrder(order: any): boolean {
    console.log("Saving order to db");
    return true
  }
}
