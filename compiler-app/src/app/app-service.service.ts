import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders, HttpResponse } from '@angular/common/http'; // Import HttpClient
import { Observable } from 'rxjs';
import { Program } from './program';
import { CustomReport } from './report';

@Injectable({
    providedIn: 'root'
})
export class AppService {

    private headers = new HttpHeaders()
        .set("Content-Type", 'application/json');

    constructor(private http: HttpClient) { }

    getResult(program: Program): Observable<HttpResponse<CustomReport>> {

        let queryParams = {};
        queryParams = {
            headers: this.headers,
            observe: "response",
        };

        return this.http.post<HttpResponse<CustomReport>>(`compiler/generate`, program, queryParams);
    }


}