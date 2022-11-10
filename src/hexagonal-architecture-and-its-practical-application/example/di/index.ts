import { Order } from "./order-core";
import { Db } from "./db-adapter";
import { MockDb } from "./db-adapter-mock";

const order1 = new Order(new Db())
order1.placeOrder()

const order2 = new Order(new MockDb())
order2.placeOrder()