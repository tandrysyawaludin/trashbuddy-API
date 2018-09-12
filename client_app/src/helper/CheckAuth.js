import * as Cookies from "js-cookie";
import { isString } from 'lodash';

export const Auth = {
  isAuthenticated: isString(Cookies.get('auth_trashbuddy')),
  authenticate(cb) {
    this.isAuthenticated = isString(Cookies.get('auth_trashbuddy'));
    setTimeout(cb, 100);
  },
  revoke(cb) {
    Cookies.remove('auth_trashbuddy');
    this.isAuthenticated = isString(Cookies.get('auth_trashbuddy'));
    setTimeout(cb, 100);
  }
};