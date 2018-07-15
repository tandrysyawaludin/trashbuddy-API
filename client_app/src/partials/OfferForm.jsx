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

import '../css/OfferForm.css';
class OfferForm extends Component {
  constructor(props) {
    super(props)
    this.state = {
      category: 1,
      area: 1
    }
  }
  
  submitToOffer = () => {
    this.props.history.push(`/home`)
  }

  changeCategory = (e) => {
    this.setState({ category: e.target.value })
  }

  render() {
    return (
      <Fragment>
        <NavbarWelcome />
        <Container className="singin-form-container">
          <Row>
            <Col md={{ size: 6, offset: 3 }}>
              <Card>
                <CardBody>
                  <CardTitle className="text-center">Sign In</CardTitle>
                  <Form>
                    <FormGroup>
                      <Label for="exampleEmail">Weight (kg)</Label>
                      <Input type="number" autocomplete={false} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Shipping Fee (Rp)</Label>
                      <Input type="text" autocomplete={false} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleEmail">Price without Shipping Fee (Rp)</Label>
                      <Input type="text" autocomplete={false} />
                    </FormGroup>
                    <FormGroup>
                      <Label for="exampleSelect">Category</Label>
                      <Input type="select" name="select" id="exampleSelect" onChange={this.changeCategory}>
                        <option>1</option>
                        <option>2</option>
                        <option>3</option>
                      </Input>
                    </FormGroup>
                    <CardText>
                      <small className="text-muted">After click "Offer" you have to wait until get a response</small>
                    </CardText>
                    <FormGroup>
                      <Button color="main" size="md" onClick={this.submitToOffer}>Offer</Button>
                      <Button color="secondary" size="md" onClick={this.props.toggleOfferForm}>Cancel</Button>                      
                    </FormGroup>
                  </Form>
                </CardBody>
              </Card>
            </Col>
          </Row>
        </Container>
      </Fragment>
    )
  }
}

export default OfferForm