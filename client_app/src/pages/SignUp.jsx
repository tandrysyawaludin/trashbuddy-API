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
  Button
} from 'reactstrap';
import { Link } from 'react-router-dom';

import NavbarWelcome from '../partials/NavbarWelcome';

import '../css/SignUp.css';
class SignUp extends Component {
  render() {
    return (
      <Fragment>
        <NavbarWelcome atSignUpPage={true} />
        <Container className="singup-form-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <Card>
                <CardBody>
                  <CardTitle className="text-center">Sign Up</CardTitle>
                  <Form>
                    <FormGroup>
                      <Label for="exampleEmail">Email</Label>
                      <Input type="email" name="email" placeholder="please input valid email" />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Password</Label>
                      <Input type="password" name="password" placeholder="input your password" />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Confirmation Password</Label>
                      <Input type="password" name="password" placeholder="input your password again" />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Full Name</Label>
                      <Input type="text" name="full-name" placeholder="input your full name" />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Phone Number</Label>
                      <Input type="text" name="phone-number" placeholder="input your active phone number" />
                    </FormGroup>
                    <CardText>
                      <small className="text-muted">Click Sign Up button is accept our <CardLink href="#">Terms and Privacy</CardLink></small>
                    </CardText>
                    <FormGroup>
                      <Button color="main" size="md" block>Sign Up</Button>
                    </FormGroup>
                  </Form>
                  <CardText>
                    <small className="text-muted">
                      <Link to="/">About</Link>
                      {} . <Link to="/">Sign In</Link>
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

export default SignUp