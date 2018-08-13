import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Navbar,
  Nav,
  NavItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import CssModules from 'react-css-modules';
import { FiHome, FiSettings, FiMail, FiRadio } from "react-icons/fi";

import styles from '../css/NavbarBottom.css';
class NavbarBottom extends Component {
  constructor(props) {
    super(props);

    this.state = {
      homePage: "",
      homeLink: "/search",
      myAdPage: "",
      myAdLink: "/my_ad",
      offerPage: "",
      offerLink: "",
      settingPage: "",
      settingLink: ""
    };
  }

  componentDidMount() { 
    if (this.props.currentRoute == "/search") {
      this.setState({
        homePage: "active-menu",
        homeLink: this.props.currentRoute
      })
    }
    else if (this.props.currentRoute == "/my_ad") {
      this.setState({
        myAdPage: "active-menu",
        postLink: this.props.currentRoute
      })
    }
  }

  render() {    
    return (
      <Navbar fixed="bottom" light expand="md" styleName="NavbarBottom">
        <Nav>
          <NavItem styleName="nav-item">
            <Link to={this.state.homeLink} styleName={this.state.homePage}><FiHome /><span>Home</span></Link>
          </NavItem>
          <NavItem styleName="nav-item">
            <Link to={this.state.myAdLink} styleName={this.state.myAdPage}><FiRadio /><span>My Ad</span></Link>
          </NavItem>
          <NavItem styleName="nav-item">
            <Link to="#" styleName={this.state.offerPage}><FiMail /><span>Offer</span></Link>
          </NavItem>
          <NavItem styleName="nav-item">
            <Link to="#" styleName={this.state.settingPage}><FiSettings /><span>Account</span></Link>
          </NavItem>
        </Nav>
      </Navbar>
    )
  }
}

export default CssModules(NavbarBottom, styles, { allowMultiple: true });
