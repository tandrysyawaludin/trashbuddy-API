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

import NavbarWelcome from '../partials/NavbarWelcome';
import '../css/SignIn.css';
class SignIn extends Component {
  constructor(props) {
    super(props);
    this.state = {
      email: "",
      password: "",
      errorSignIn: false,
      errorSignInMessage: ""
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
        this.props.history.push(`/home`);
      }
      else {
        this.setState({ errorSignIn: true, errorSignInMessage: "Email and password do not match" });  
        console.log('salah')    
      }
      console.log('sukses', response);
    })
    .catch(error => {
      // handle error
      this.setState({ errorSignIn: true, errorSignInMessage: "Sorry, our system is busy now :(" });
      console.log('gagal', error);
    })
    .then(() => {
      // always executed
      console.log('selalu');      
    });
  }

  onDismiss() {
    this.setState({ errorSignIn: false });
  }

  render() {
    return (
      <Fragment>
        <NavbarWelcome />
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
                      <Button color="main" size="md" block onClick={this.handleSubmit} type="submit">Sign In</Button>
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
      </Fragment>
    )
  }
}

export default SignIn