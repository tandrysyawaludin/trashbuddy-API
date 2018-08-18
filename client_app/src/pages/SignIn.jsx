import React, { Component, Fragment } from 'react';
import {
  Container,
  Row,
  Col,
  Card,
  CardTitle,
  CardBody,
  CardText,
  CardLink,
  Form,
  FormGroup,
  Label,
  Input,
  Button,
  Alert
} from 'reactstrap';
import { Link } from 'react-router-dom';
import axios from 'axios';
import CSSModules from 'react-css-modules';
import * as Cookies from "js-cookie";
import _ from 'lodash';

import NavbarWelcome from '../partials/NavbarWelcome';
import styles from '../css/SignIn.css';
import loader from '../img/loader.svg';

class SignIn extends Component {
  constructor(props) {
    super(props);
    this.state = {
      email: "",
      password: "",
      errorSignIn: false,
      errorSignInMessage: "",
      submitting: false,
    };

    this.handleInputChange = this.handleInputChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
    this.onDismiss = this.onDismiss.bind(this);
  }

  handleInputChange(event) {
    const value = event.target.value;
    const name = event.target.name;

    this.setState({
      [name]: value
    });
  }

  handleSubmit(event) { 
    event.preventDefault();
    event.stopPropagation();

    this.setState({ submitting: true });

    let data = {
      email: this.state.email,
      password: this.state.password
    }

    let headers = {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    }    

    axios({
      method: 'POST',
      url: 'http://localhost:8000/supplier/auth',
      headers: headers,
      data: data
    })
    .then(response => {
      if (response.data.success === true) {
        Cookies.set('auth_trashbuddy', response.data.jwt, { expires: 1 });
        this.props.history.push('/home');             
      }
      else {
        this.setState({ errorSignIn: true, errorSignInMessage: "Email and password do not match" });  
        let cookies = Cookies.get();
        _.mapKeys(cookies, (val, key) => {        
          Cookies.remove(key);
        });
      }
    })
    .catch(error => {
      this.setState({ errorSignIn: true, errorSignInMessage: "Sorry, our system is busy now :(" });
      let cookies = Cookies.get();
      _.mapKeys(cookies, (val, key) => {
        Cookies.remove(key);
      });     
    })
    .then(() => {
      this.setState({ submitting: false });     
    });
  }

  onDismiss() {
    this.setState({ errorSignIn: false });
  }

  render() {
    return (
      <Fragment>
        <NavbarWelcome />
        <div styleName="SignIn">
          <Container className="singin-form-container">
            <Row>
              <Col md={{ size: 6, offset: 3 }}>
                <Alert color="danger" isOpen={this.state.errorSignIn} toggle={this.onDismiss}>
                  {this.state.errorSignInMessage}
                </Alert>
                <Card>
                  <CardBody>
                    <CardTitle className="text-center">Sign In</CardTitle>
                    <Form>
                      <FormGroup>
                        <Label for="exampleEmail">Email</Label>
                        <Input type="email" name="email" placeholder="please input valid email" onChange={this.handleInputChange} />
                      </FormGroup>
                      <FormGroup>
                        <Label for="exampleEmail">Password</Label>
                        <Input type="password" name="password" 
                          placeholder="input your password" onChange={this.handleInputChange}  />
                      </FormGroup>
                      <CardText>
                        <small className="text-muted">Click Sign In button is accept our <CardLink href="#">Terms and Privacy</CardLink></small>
                      </CardText>
                      <FormGroup>
                        <Button color="main" size="md" block onClick={this.handleSubmit} type="submit">
                          { this.state.submitting ? <img src={loader} /> : "Sign In" }
                        </Button>
                      </FormGroup>
                    </Form>
                    <CardText>
                      <small className="text-muted">
                        <Link to="/">About</Link>
                        {} . <Link to="/">Forgot Password?</Link>
                        {} . <Link to="/sign_up">Sign Up</Link>
                      </small>
                    </CardText>
                  </CardBody>
                </Card>
              </Col>
            </Row>
          </Container>
        </div>
      </Fragment>
    )
  }
}

export default CSSModules(SignIn, styles, { allowMultiple: true });
