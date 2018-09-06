import * as Cookies from "js-cookie";
import { isString } from 'lodash';

export const Auth = {
  isAuthenticated: isString(Cookies.get('auth_trashbuddy')),
  authenticate(cb) {
    this.isAuthenticated = isString(Cookies.get('auth_trashbuddy'));
    setTimeout(cb, 100);
  }
};