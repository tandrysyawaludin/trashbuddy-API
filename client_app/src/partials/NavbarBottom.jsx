import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Collapse,
  Navbar,
  NavbarToggler,
  NavbarBrand,
  Nav,
  NavItem,
  NavLink,
  UncontrolledDropdown,
  DropdownToggle,
  DropdownMenu,
  DropdownItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import TiHome from 'react-icons/lib/ti/home';
import MdAddCircle from 'react-icons/lib/md/add-circle';
import MdSettings from 'react-icons/lib/md/settings';
import MdAssignment from 'react-icons/lib/md/assignment';

import '../css/NavbarBottom.css';
class NavbarBottom extends Component {
  constructor(props) {
    super(props);

    this.toggle = this.toggle.bind(this);
    this.state = {
      isOpen: false
    };
  }
  toggle() {
    this.setState({
      isOpen: !this.state.isOpen
    });
  }
  render() {
    return (
      <Fragment>
        <Navbar fixed="bottom" light expand="md" className="navbar-bottom navbar-default">
          <Nav>
            <NavItem>
              <NavLink href="#"><TiHome /><span>Home</span></NavLink>
            </NavItem>
            <NavItem>
              <NavLink href="#"><MdAddCircle /><span>Post</span></NavLink>
            </NavItem>
            <NavItem>
              <NavLink href="#"><MdAssignment /><span>Order</span></NavLink>
            </NavItem>
            <NavItem>
              <NavLink href="#"><MdSettings /><span>Account</span></NavLink>
            </NavItem>
          </Nav>
        </Navbar>
      </Fragment>
    )
  }
}

export default NavbarBottom